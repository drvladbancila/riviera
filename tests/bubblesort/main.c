#define N 6

void swap(int *a, int *b);
void bubbleSort(int v[], int size);

int main()
{
    int v[N] = {5, 7, 2, 1, 8, 10};
    int i;

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