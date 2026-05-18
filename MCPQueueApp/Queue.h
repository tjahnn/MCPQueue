#pragma once
#include <stdexcept>
#include <vector>

template <typename T>
class Queue {
private:
    struct Node {
        T data;
        Node* next;
        Node(const T& value) : data(value), next(nullptr) {}
    };

    Node* head;
    Node* tail;
    int count;

public:
    Queue() : head(nullptr), tail(nullptr), count(0) {}

    ~Queue() {
        while (!isEmpty())
            dequeue();
    }

    void enqueue(const T& value) {
        Node* newNode = new Node(value);
        if (tail)
            tail->next = newNode;
        tail = newNode;
        if (!head)
            head = newNode;
        ++count;
    }

    T dequeue() {
        if (isEmpty())
            throw std::underflow_error("Queue is empty");
        T value = head->data;
        Node* temp = head;
        head = head->next;
        if (!head)
            tail = nullptr;
        delete temp;
        --count;
        return value;
    }

    const T& peek() const {
        if (isEmpty())
            throw std::underflow_error("Queue is empty");
        return head->data;
    }

    std::vector<T> getAll() const {
        std::vector<T> result;
        result.reserve(count);
        Node* current = head;
        while (current) {
            result.push_back(current->data);
            current = current->next;
        }
        return result;
    }

    void clear() {
        while (!isEmpty())
            dequeue();
    }

    bool isEmpty() const { return count == 0; }
    int size() const { return count; }
};
