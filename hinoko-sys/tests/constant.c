// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    float: "%f", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT((gint) HINOKO_FW_ISO_CTX_ERROR_ALLOCATED);
    PRINT_CONSTANT((gint) HINOKO_FW_ISO_CTX_ERROR_CHUNK_UNREGISTERED);
    PRINT_CONSTANT((gint) HINOKO_FW_ISO_CTX_ERROR_FAILED);
    PRINT_CONSTANT((gint) HINOKO_FW_ISO_CTX_ERROR_MAPPED);
    PRINT_CONSTANT((gint) HINOKO_FW_ISO_CTX_ERROR_NOT_ALLOCATED);
    PRINT_CONSTANT((gint) HINOKO_FW_ISO_CTX_ERROR_NOT_MAPPED);
    PRINT_CONSTANT((gint) HINOKO_FW_ISO_CTX_ERROR_NO_ISOC_CHANNEL);
    PRINT_CONSTANT((guint) HINOKO_FW_ISO_CTX_MATCH_FLAG_TAG0);
    PRINT_CONSTANT((guint) HINOKO_FW_ISO_CTX_MATCH_FLAG_TAG1);
    PRINT_CONSTANT((guint) HINOKO_FW_ISO_CTX_MATCH_FLAG_TAG2);
    PRINT_CONSTANT((guint) HINOKO_FW_ISO_CTX_MATCH_FLAG_TAG3);
    PRINT_CONSTANT((gint) HINOKO_FW_ISO_CTX_MODE_RX_MULTIPLE);
    PRINT_CONSTANT((gint) HINOKO_FW_ISO_CTX_MODE_RX_SINGLE);
    PRINT_CONSTANT((gint) HINOKO_FW_ISO_CTX_MODE_TX);
    PRINT_CONSTANT((gint) HINOKO_FW_ISO_RESOURCE_AUTO_ERROR_ALLOCATED);
    PRINT_CONSTANT((gint) HINOKO_FW_ISO_RESOURCE_AUTO_ERROR_FAILED);
    PRINT_CONSTANT((gint) HINOKO_FW_ISO_RESOURCE_AUTO_ERROR_NOT_ALLOCATED);
    PRINT_CONSTANT((gint) HINOKO_FW_ISO_RESOURCE_AUTO_ERROR_TIMEOUT);
    PRINT_CONSTANT((gint) HINOKO_FW_ISO_RESOURCE_ERROR_EVENT);
    PRINT_CONSTANT((gint) HINOKO_FW_ISO_RESOURCE_ERROR_FAILED);
    PRINT_CONSTANT((gint) HINOKO_FW_ISO_RESOURCE_ERROR_NOT_OPENED);
    PRINT_CONSTANT((gint) HINOKO_FW_ISO_RESOURCE_ERROR_OPENED);
    PRINT_CONSTANT((gint) HINOKO_FW_ISO_RESOURCE_ERROR_TIMEOUT);
    PRINT_CONSTANT((gint) HINOKO_FW_SCODE_S100);
    PRINT_CONSTANT((gint) HINOKO_FW_SCODE_S1600);
    PRINT_CONSTANT((gint) HINOKO_FW_SCODE_S200);
    PRINT_CONSTANT((gint) HINOKO_FW_SCODE_S3200);
    PRINT_CONSTANT((gint) HINOKO_FW_SCODE_S400);
    PRINT_CONSTANT((gint) HINOKO_FW_SCODE_S800);
    return 0;
}
