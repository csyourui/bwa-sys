#include "bwa/bntseq.h"
#include "bwa/bwa.h"
#include "bwa/bwamem.h"
#include "bwa/bwt.h"
#include "bwa/kbtree.h"
#include "bwa/khash.h"
#include "bwa/kseq.h"
#include "bwa/ksort.h"
#include "bwa/kstring.h"
#include "bwa/ksw.h"
#include "bwa/kvec.h"
#include "bwa/malloc_wrap.h"
#include "bwa/utils.h"

#include <zlib.h>
KSEQ_DECLARE(gzFile)