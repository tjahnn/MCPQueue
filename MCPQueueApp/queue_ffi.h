#pragma once
#include <cstddef>

#ifdef __cplusplus
extern "C" {
#endif

typedef void* QueueHandle;

QueueHandle queue_create();
void        queue_destroy(QueueHandle handle);

void  queue_enqueue(QueueHandle handle, const char* json);
char* queue_dequeue(QueueHandle handle);   // NULL if empty
char* queue_peek   (QueueHandle handle);   // NULL if empty
char* queue_get_all(QueueHandle handle);   // JSON array string
size_t queue_size  (QueueHandle handle);
void   queue_clear (QueueHandle handle);

void queue_free_string(char* ptr);

#ifdef __cplusplus
}
#endif
