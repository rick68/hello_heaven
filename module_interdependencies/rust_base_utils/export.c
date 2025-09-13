#include <linux/module.h>

extern void hello(const unsigned char* name);
EXPORT_SYMBOL_GPL(hello);
