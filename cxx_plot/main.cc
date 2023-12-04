#include <dora-node-api.h>
#include <iostream>

int main() {
    auto dora_node = init_dora_node();

    for (;;) {
        auto event = next_event(dora_node.events);
        auto ty = event_type(event);

        if (ty == DoraEventType::AllInputsClosed) {
            break;
        }
        else if (ty == DoraEventType::Input) {
            auto input = event_as_input(std::move(event));

            std::cout << "Received input " << std::string(input.id) << std::endl;
        }
        else
        {
            std::cerr << "Unknown event type " << static_cast<int>(ty) << std::endl;
            break;
        }
    }

    return 0;
}
