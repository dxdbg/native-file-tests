/*
 * Copyright (c) 2011-2017, UDI Contributors
 * All rights reserved.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

package net.libudi.nativefiletests;

import java.util.Map;
import java.util.Optional;

/**
 * Java object for a native file tests JSON file
 */
public class NativeFileMetadata
{
    private String configName;

    private String baseName;

    private String objectSuffix;

    private Map<String, String> objectSha256s;

    private String strippedSha256;

    private String executableSuffix;

    private String executableSha256;

    private String debugSha256;

    private String machine;

    private String platform;

    private Map<String, String> flags;

    private String compiler;

    public String getConfigName()
    {
        return configName;
    }

    public void setConfigName(String configName)
    {
        this.configName = configName;
    }

    public String getBaseName()
    {
        return baseName;
    }

    public void setBaseName(String baseName)
    {
        this.baseName = baseName;
    }

    public Map<String, String> getObjectSha256s()
    {
        return objectSha256s;
    }

    public void setObjectSha256s(Map<String, String> objectSha256s)
    {
        this.objectSha256s = objectSha256s;
    }

    public String getExecutableSha256()
    {
        return executableSha256;
    }

    public void setExecutableSha256(String executableSha256)
    {
        this.executableSha256 = executableSha256;
    }

    public Optional<String> getDebugSha256()
    {
        return Optional.ofNullable(debugSha256);
    }

    public void setDebugSha256(String debugSha256)
    {
        this.debugSha256 = debugSha256;
    }

    public String getStrippedSha256()
    {
        return strippedSha256;
    }

    public void setStrippedSha256(String strippedSha256)
    {
        this.strippedSha256 = strippedSha256;
    }

    public String getMachine()
    {
        return machine;
    }

    public void setMachine(String machine)
    {
        this.machine = machine;
    }

    public String getPlatform()
    {
        return platform;
    }

    public void setPlatform(String platform)
    {
        this.platform = platform;
    }

    public Map<String, String> getFlags()
    {
        return flags;
    }

    public void setFlags(Map<String, String> flags)
    {
        this.flags = flags;
    }

    public String getCompiler()
    {
        return compiler;
    }

    public void setCompiler(String compiler)
    {
        this.compiler = compiler;
    }

    public String getObjectSuffix()
    {
        return objectSuffix;
    }

    public void setObjectSuffix(String objectSuffix)
    {
        this.objectSuffix = objectSuffix;
    }

    public String getExecutableSuffix()
    {
        return executableSuffix;
    }

    public void setExecutableSuffix(String executableSuffix)
    {
        this.executableSuffix = executableSuffix;
    }
}
