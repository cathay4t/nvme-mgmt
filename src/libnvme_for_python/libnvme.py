# Copyright (C) 2017 Red Hat, Inc.
# This library is free software; you can redistribute it and/or
# modify it under the terms of the GNU Lesser General Public
# License as published by the Free Software Foundation; either
# version 2.1 of the License, or (at your option) any later version.
#
# This library is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
# Lesser General Public License for more details.
#
# You should have received a copy of the GNU Lesser General Public
# License along with this library; If not, see <http://www.gnu.org/licenses/>.
#
# Author: Gris Ge <fge@redhat.com>

import ctypes
import unittest
from ctypes import c_int, c_char_p, c_uint32, Structure, POINTER, byref


class _NvmeCtrlC(Structure):
    pass


lib = ctypes.cdll.LoadLibrary("libnvme.so")

lib.nvme_ctrl_get.restype = c_int
lib.nvme_ctrl_get.argtypes = (c_char_p, POINTER(POINTER(_NvmeCtrlC)),
                              POINTER(c_char_p))

lib.nvme_ctrl_free.restype = None
lib.nvme_ctrl_free.argtypes = (POINTER(_NvmeCtrlC),)

lib.nvme_ctrl_sn_get.restype = c_char_p
lib.nvme_ctrl_sn_get.argtypes = (POINTER(_NvmeCtrlC),)


class NvmeError(Exception):
    NVME_OK = 0
    NVME_LIBBUG = 1

    def __init__(self, errno, message, *args, **kwargs):
        Exception.__init__(self, *args, **kwargs)
        self.errno = errno
        self.message = message

    def __str__(self):
        return "Error %s: %s" % (self.errno, self.message)


class NvmeCtrl(object):
    def __init__(self, dev_path):
        c_obj = POINTER(_NvmeCtrlC)()
        err_msg = c_char_p()
        rc = lib.nvme_ctrl_get(dev_path, byref(c_obj), byref(err_msg))
        if rc != NvmeError.NVME_OK:
            raise NvmeError(rc, err_msg.value)
        self.sn = lib.nvme_ctrl_sn_get(c_obj)
        self._c_obj = c_obj

    def __exit__(self, exc_type, exc_value, traceback):
        lib.nvme_ctrl_free(self._c_obj)


class TestCommon(unittest.TestCase):
    def setUp(self):
        pass

    def test_simple(self):
        obj = NvmeCtrl("/dev/nvme0")
        print("SN: '%s'" % obj.sn)

    def tearDown(self):
        pass


if __name__ == '__main__':
    unittest.main()
