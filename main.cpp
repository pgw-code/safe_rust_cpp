#include <iostream>
#include <string>
#include <cstring>
#include <memory>

extern "C" {
    void linked_list_create(int id);
    void linked_list_add_at_beginning(int id, int value);
    void linked_list_add_at_end(int id, int value);
    void linked_list_remove_from_beginning(int id);
    void linked_list_remove_from_end(int id);
    char* linked_list_print(int id);
    void linked_list_free_string(char* ptr);
}

class LinkedListWrapper {
    int id;
public:
    LinkedListWrapper(int list_id) : id(list_id) {
        linked_list_create(id);
    }

    void addAtBeginning(int value) {
        linked_list_add_at_beginning(id, value);
    }

    void addAtEnd(int value) {
        linked_list_add_at_end(id, value);
    }

    void removeFromBeginning() {
        linked_list_remove_from_beginning(id);
    }

    void removeFromEnd() {
        linked_list_remove_from_end(id);
    }

    void printList() {
        char* result = linked_list_print(id);
        if (result) {
            std::cout << result << std::endl;
            linked_list_free_string(result);
        }
    }
};

int main() {
    LinkedListWrapper list(1);

    list.addAtEnd(10);
    list.addAtEnd(20);
    list.addAtBeginning(5);
    list.printList();

    list.removeFromEnd();
    list.printList();

    list.removeFromBeginning();
    list.printList();

    return 0;
}
