#include <iostream>
#include <unordered_map>
#include <vector>

int s(int r, std::vector<int> i)
{
    std::vector<int> m (r);
    int c = 1;
    for (unsigned n = 0; n < i.size(); c++,n++)
        m[i[n]] = c;

    int l = i.back(),u = 0;
    for (; c <= r; c++) {
        l = u;
        if (m[u] != 0) {
            u = c - m[u];
        } else {
            u = 0;
        }
        m[l] = c;
    }

    return l;
}

int main(void)
{
    auto i = {1,0,18,10,19,6};
    printf("%d\n",s(2020, i));
    printf("%d\n",s(30000000, i));

    return 0;
}
