#include <bpx/container.h>
#include <bpx/section.h>
#include <bpx/error_codes.h>

#include <stdio.h>
#include <assert.h>

static void print_main_header(const bpx_container_t *container)
{
    bpx_main_header_t header;
    bpx_container_get_main_header(container, &header);
    printf("-- BPX main header --\n");
    printf("%.3s Type %c, version: %d\n", header.signature, header.ty, header.version);
    printf("Header checksum: %d\n", header.chksum);
    printf("File size: %ld\n", header.file_size);
    printf("Number of sections: %d\n", header.section_num);
    printf("-- END --\n");
}

int main(int ac, const char **av)
{
    if (ac != 2)
        return 1;
    bpx_container_t *container;
    bpx_error_t err = bpx_container_open(av[1], &container);
    if (err != BPX_ERR_NONE)
    {
        fprintf(stderr, "Couldn't open BPX container, error: %d\n", err);
        return 1;
    }
    assert(container != NULL);
    print_main_header(container);
    bpx_container_close(&container);
    assert(container == NULL);
    return 0;
}
