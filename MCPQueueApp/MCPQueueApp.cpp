// MCPQueueApp.cpp : 이 파일에는 'main' 함수가 포함됩니다. 거기서 프로그램 실행이 시작되고 종료됩니다.

#include <iostream>
#include "Queue.h"

int main()
{
    Queue<int> q;

    std::cout << "=== Queue 예제 ===\n\n";

    for (int i = 1; i <= 5; ++i) {
        q.enqueue(i * 10);
        std::cout << "enqueue: " << i * 10 << "  (size=" << q.size() << ")\n";
    }

    std::cout << "\nfront: " << q.peek() << "\n\n";

    while (!q.isEmpty()) {
        std::cout << "dequeue: " << q.dequeue() << "  (size=" << q.size() << ")\n";
    }

    try {
        q.dequeue();
    }
    catch (const std::underflow_error& e) {
        std::cout << "\n예외 발생: " << e.what() << "\n";
    }

    return 0;
}
