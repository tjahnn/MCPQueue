#include "queue_ffi.h"
#include "Queue.h"
#include <string>
#include <cstring>

using CppQueue = Queue<std::string>;

extern "C" {

QueueHandle queue_create() {
    return new CppQueue();
}

void queue_destroy(QueueHandle handle) {
    delete static_cast<CppQueue*>(handle);
}

void queue_enqueue(QueueHandle handle, const char* json) {
    static_cast<CppQueue*>(handle)->enqueue(std::string(json));
}

static char* dup_str(const std::string& s) {
    char* buf = new char[s.size() + 1];
    std::memcpy(buf, s.c_str(), s.size() + 1);
    return buf;
}

char* queue_dequeue(QueueHandle handle) {
    auto* q = static_cast<CppQueue*>(handle);
    if (q->isEmpty()) return nullptr;
    return dup_str(q->dequeue());
}

char* queue_peek(QueueHandle handle) {
    auto* q = static_cast<CppQueue*>(handle);
    if (q->isEmpty()) return nullptr;
    return dup_str(q->peek());
}

char* queue_get_all(QueueHandle handle) {
    auto* q = static_cast<CppQueue*>(handle);
    auto items = q->getAll();
    std::string json = "[";
    for (size_t i = 0; i < items.size(); ++i) {
        if (i > 0) json += ',';
        json += items[i];
    }
    json += ']';
    return dup_str(json);
}

size_t queue_size(QueueHandle handle) {
    return static_cast<CppQueue*>(handle)->size();
}

void queue_clear(QueueHandle handle) {
    static_cast<CppQueue*>(handle)->clear();
}

void queue_free_string(char* ptr) {
    delete[] ptr;
}

} // extern "C"
