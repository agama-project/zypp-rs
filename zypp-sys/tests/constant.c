// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
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
    PRINT_CONSTANT((gint) ZYPP_ERROR);
    PRINT_CONSTANT((gint) ZYPP_REPO_MANAGER_ERROR_REF_ABORTED);
    PRINT_CONSTANT((gint) ZYPP_REPO_MANAGER_ERROR_REF_FAILED);
    PRINT_CONSTANT((gint) ZYPP_REPO_MANAGER_ERROR_REF_SKIPPED);
    PRINT_CONSTANT((gint) ZYPP_REPO_MANAGER_REFRESHED);
    PRINT_CONSTANT((gint) ZYPP_REPO_MANAGER_UP_TO_DATE);
    PRINT_CONSTANT((gint) ZYPP_REPO_NONE);
    PRINT_CONSTANT((gint) ZYPP_REPO_RPMMD);
    PRINT_CONSTANT((gint) ZYPP_REPO_RPMPLAINDIR);
    PRINT_CONSTANT((gint) ZYPP_REPO_YAST2);
    return 0;
}
