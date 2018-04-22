/*
 * Copyright (c) 2011-2018, UDI Contributors
 * All rights reserved.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

package net.libudi.nativefiletests;

import java.io.IOException;
import java.nio.file.Paths;

import org.junit.Test;

import static org.junit.Assert.assertNotNull;

/**
 * Unit test for {@link NativeFileTestsInfo}
 */
public class NativeFileTestsInfoTest
{
    private static final String NATIVE_FILE_TEST_PATH = "native.file.tests.basePath";
    private static final String SIMPLE_EXEC_NAME = "simple-debug-noopt-dynamic";

    private final String basePath;

    public NativeFileTestsInfoTest()
    {
        basePath = System.getProperty(NATIVE_FILE_TEST_PATH);
        assertNotNull(basePath);
    }

    @Test
    public void testLoadSimpleInfo() throws IOException
    {
        NativeFileTestsInfo info = new NativeFileTestsInfo(Paths.get(basePath));
        info.getFirstExecutablePath(SIMPLE_EXEC_NAME);
    }
}
