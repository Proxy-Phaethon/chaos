#ifndef RESOLVER_H
#define RESOLVER_H

typedef struct
{
    const char *conditions[16];
    int count;
} Resolution;

Resolution resolve(const char *output);

#endif