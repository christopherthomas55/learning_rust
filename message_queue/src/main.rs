use rust_book::ch17_async::*;
use super;


fn main() {
    queues = vec!(queue);
    PREV = None;
    // Can add a message to any queue with a hashmap
    message = Message{0, "test", "success?", PREV};
    add_message_into(message, queues); // queue is not global so add_message is a weird name

    // Can list all of them

    list_message_queues(queues);

    // get any of them
    // TODO - Use option here? Is it working
    if let Some(result) = from_queue_pop(queues, 0){ // TODO Eventually do, mark_read, delete);
        format!("Success - {result}");
    } else {
        format!("Failure - {result}");
    }


    

    // And thats it :P
}


