#include <iostream>
#include <unordered_map>
#include <vector>

int s(int r, std::vector<int> i)
{
    std::unordered_map<int, int> m;
    for (unsigned n = 0; n < i.size(); n++)
        m[i[n]] = n;

    int l,u = 0;
    for (int c = i.size(); c < r; c++) {
        l = u;
        if (m.count(u)) {
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
