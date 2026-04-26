#include <iostream>
#include <vector>
#include <string>

int main() {
    std::vector<std::string> names = {"Alice", "Bob", "Charlie"};
    
    auto it = names.begin();
    names.push_back("Dave");

    std::cout << *it << "\n";
    return 0;
}