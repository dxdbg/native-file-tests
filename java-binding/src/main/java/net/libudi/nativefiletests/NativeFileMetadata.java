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
    private String baseName;

    private Map<String, String> objectSha1s;

    private String strippedSha1;

    private String executableSha1;

    private String debugSha1;

    private String machine;

    private String platform;

    private Map<String, String> flags;

    private String compiler;

    public String getBaseName()
    {
        return baseName;
    }

    public void setBaseName(String baseName)
    {
        this.baseName = baseName;
    }

    public Map<String, String> getObjectSha1s()
    {
        return objectSha1s;
    }

    public void setObjectSha1s(Map<String, String> objectSha1s)
    {
        this.objectSha1s = objectSha1s;
    }

    public String getExecutableSha1()
    {
        return executableSha1;
    }

    public void setExecutableSha1(String executableSha1)
    {
        this.executableSha1 = executableSha1;
    }

    public Optional<String> getDebugSha1()
    {
        return Optional.ofNullable(debugSha1);
    }

    public void setDebugSha1(String debugSha1)
    {
        this.debugSha1 = debugSha1;
    }

    public String getStrippedSha1()
    {
        return strippedSha1;
    }

    public void setStrippedSha1(String strippedSha1)
    {
        this.strippedSha1 = strippedSha1;
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
}
