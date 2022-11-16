#include "../lfsr/lfsr.h"
#define N 1000

void swap(int *a, int *b);
void bubbleSort(int v[], int size);

int _start()
{
    int v[N];
    int i;

    /* Generate N random numbers */
    for (i = 0; i < N; i++) {
        v[i] = lfsr();
    }

    /* Sort the N random numbers */
    bubbleSort(v, N);

    return 0;
}

void swap(int *a, int *b)
{
    int tmp;
    tmp = *a;
    *a = *b;
    *b = tmp;
}

void bubbleSort(int v[], int size)
{
    int unsorted = 1;
    int i, swaps;

    while (unsorted) {
        swaps = 0;
        
        for (i = 0; i < size - 1; i++) {
            if (v[i] > v[i + 1]) {
                swap(&v[i], &v[i + 1]);
                swaps = 1;
            }
        }

        if (!swaps) {
            unsorted = 0;
        }
    }

}