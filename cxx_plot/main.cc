#include <dora-node-api.h>
#include <iostream>
#include <arrow/array.h>
#include <arrow/api.h>
#include <arrow/io/memory.h>
#include <arrow/ipc/reader.h>
#include <arrow/ipc/api.h>
#include <arrow/util/logging.h>

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
            if (std::string(input.id) == "stereo_image") {
                auto data_ptr = input.data.data();
                auto size = input.data.size();
                std::shared_ptr<arrow::Buffer> buffer = arrow::Buffer::Wrap(data_ptr, size);

                std::vector<std::shared_ptr<arrow::Field>> fields;
                std::shared_ptr<arrow::Field> width =  arrow::field("width", arrow::uint32());
                std::shared_ptr<arrow::Field> height = arrow::field("height", arrow::uint32());
                std::shared_ptr<arrow::Field> raw_data = arrow::field("raw", arrow::fixed_size_list(arrow::uint8(), 453620));
                fields.push_back(width);
                fields.push_back(height);
                fields.push_back(raw_data);
                std::shared_ptr<arrow::DataType> struct_type = arrow::struct_(fields);
                std::shared_ptr<arrow::ArrayData> array_data = arrow::ArrayData::Make(
                    struct_type,
                    2,
                    {buffer, buffer, buffer}
                );

                arrow::StructArray struct_array{array_data};
                arrow::UInt32Array width_array{struct_array.GetFieldByName("width")->data()};
                // arrow::UInt32Array width_array{struct_array.field(0)->data()};

            } else {
                std::cerr << "Unexpected input " << std::string(input.id) << std::endl;
                break;
            }
        }
        else
        {
            std::cerr << "Unknown event type " << static_cast<int>(ty) << std::endl;
            break;
        }
    }

    return 0;
}
