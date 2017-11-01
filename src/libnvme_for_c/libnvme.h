/*
 * Copyright (C) 2017 Red Hat, Inc.
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; If not, see <http://www.gnu.org/licenses/>.
 *
 * Author: Gris Ge <fge@redhat.com>
 */

#ifndef _LIBNVME_H_
#define _LIBNVME_H_

#ifdef __cplusplus
extern "C" {
#endif

#define NVME_OK				0
#define NVME_LIBBUG			1

struct nvme_ctrl;

int nvme_ctrl_get(const char *dev_path, struct nvme_ctrl **cnt, char **err_msg);

void nvme_ctrl_free(struct nvme_ctrl *cnt);

const char *nvme_ctrl_sn_get(struct nvme_ctrl *cnt);

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* End of _LIBNVME_H_ */
