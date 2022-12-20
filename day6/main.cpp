#include <string>
#include <unordered_map>
#include <fstream>
#include <iostream>
#include <vector>

std::vector<std::string> split_into_all_possible_fourteen_letter_substring(const std::string &s) {
    std::vector<std::string> substrings;
    for (auto i = 0; i < s.size() - 13; ++i) {
        substrings.push_back(s.substr(i, 14));
    }
    return substrings;
}

int index_of_first_substring_with_fourteen_different_letters(const std::vector<std::string> &substrings) {
    for (auto i = 0; i < substrings.size(); ++i) {
        auto substring = substrings[i];
        std::unordered_map<char, int> letter_count;
        for (auto c : substring) {
            letter_count[c]++;
        }
        if (letter_count.size() == 14) {
            return i;
        }
    }
    return -1;
}

std::string read_in_file_called_input() {
    std::ifstream input("input");
    std::string s;
    input >> s;
    return s;
}

int main() {
    auto input = read_in_file_called_input();
    std::cout << "Input is: " << input << std::endl;
    auto substrings = split_into_all_possible_fourteen_letter_substring(input);
    auto index = index_of_first_substring_with_fourteen_different_letters(substrings);
    std::cout << "Index of first substring with four different letters is: " << index+14 << std::endl;
}
