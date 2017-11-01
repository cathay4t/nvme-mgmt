#include <stdio.h>
#include <stdlib.h>
#include <libnvme.h>

int main(void)
{
	struct nvme_ctrl *cnt = NULL;
	int rc = NVME_OK;
	char *err_msg = NULL;

	rc = nvme_ctrl_get("/dev/nvme0", &cnt, &err_msg);
	if (rc != NVME_OK) {
		printf("erro %d: '%s'\n", rc, err_msg);
		exit(1);
	}
	printf("SN: '%s'\n", nvme_ctrl_sn_get(cnt));
	nvme_ctrl_free(cnt);
}
