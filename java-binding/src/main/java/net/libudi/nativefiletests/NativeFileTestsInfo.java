/*
 * Copyright (c) 2011-2017, UDI Contributors
 * All rights reserved.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

package net.libudi.nativefiletests;

import java.io.IOException;
import java.io.InputStream;
import java.nio.file.DirectoryStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.Collection;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.stream.Collectors;

import com.fasterxml.jackson.databind.ObjectMapper;

import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertTrue;

/**
 * Container for native-file-tests metadata
 */
public final class NativeFileTestsInfo
{
    private static final ObjectMapper objectMapper = new ObjectMapper();
    private final Map<String, Set<Path>> objectPaths = new HashMap<>();
    private final Map<String, Set<Path>> executablePaths = new HashMap<>();

    public NativeFileTestsInfo(Path basePath) throws IOException
    {
        String platformName = getNftPlatformName();

        try (DirectoryStream<Path> stream = Files.newDirectoryStream(basePath, "*.json.*")) {
            for (Path jsonFile : stream)
            {
                String baseName = jsonFile.getFileName().toString().split("\\.")[0];

                try (InputStream jsonFileStream = Files.newInputStream(jsonFile)) {
                    NativeFileMetadata metadata = objectMapper.readValue(jsonFileStream, NativeFileMetadata.class);

                    if (metadata.getPlatform().equals(platformName)) {
                        Set<Path> objectFilePaths = objectPaths.computeIfAbsent(baseName, k -> new HashSet<>());

                        objectFilePaths.addAll(metadata.getObjectSha1s()
                                                       .entrySet()
                                                       .stream()
                                                       .map(e -> e.getKey() + "." + e.getValue())
                                                       .map(file -> Paths.get(basePath.toAbsolutePath().toString(),
                                                                              file))
                                                       .collect(Collectors.toList()));

                        String executableFileName = baseName + "." + metadata.getDebugSha1()
                                                                             .map(s -> "debug." + s)
                                                                             .orElse(metadata.getExecutableSha1());

                        Set<Path> executableFilePaths = executablePaths.computeIfAbsent(baseName, k -> new HashSet<>());
                        executableFilePaths.add(Paths.get(basePath.toAbsolutePath().toString(), executableFileName));
                    }
                }
            }
        }

        if (executablePaths.size() == 0)
        {
            throw new IllegalStateException("Failed to locate executables for " + platformName);
        }
    }

    public Path getFirstObjectPath(String objectFileName)
    {
        Set<Path> objectFilePaths = objectPaths.get(objectFileName);
        assertNotNull(objectFilePaths);
        assertTrue(objectFilePaths.size() > 0);
        return objectFilePaths.iterator().next();
    }

    public Path getFirstExecutablePath(String executableFileName)
    {
        Set<Path> executableFilePaths = executablePaths.get(executableFileName);
        assertNotNull(executableFilePaths);
        assertTrue(executableFilePaths.size() > 0);
        return executableFilePaths.iterator().next();
    }

    public List<Path> getObjectPaths()
    {
        return objectPaths.values()
                          .stream()
                          .flatMap(Collection::stream)
                          .collect(Collectors.toList());
    }

    public List<Path> getExecutablePaths()
    {
        return executablePaths.values()
                              .stream()
                              .flatMap(Collection::stream)
                              .collect(Collectors.toList());
    }

    private static String getNftPlatformName()
    {
        String osName = System.getProperty("os.name");

        if (osName.equalsIgnoreCase("Linux")) {
            return "linux";
        }

        if (osName.contains("mac")) {
            return "darwin";
        }

        throw new IllegalStateException("Unsupported OS " + osName);
    }
}
