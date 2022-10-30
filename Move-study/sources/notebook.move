//一个简单记事本 实现增删改查的功能
address 0x1 { 
    module notebook{   
        use 0x1::Debug; 
        use 0x1::Vector;
        use 0x1::Signer; 

        struct Event has copy,store,drop {
            words: vector<u8>,
            time: u64,
            encrypt: bool,
        }

        //Notebook has vector of event
        struct Notebook has key, drop {
            value:vector<Event>,
        }

        //init function move notebook to signer
        public fun init(account: & signer) {
            move_to<Notebook>(account, Notebook { 
                value: Vector::empty<Event>() 
            });
        }

        // add event
        public fun addEvent(account: &signer, newevent:Event) acquires Notebook {
            let notebook = borrow_global_mut<Notebook>(Signer::address_of(account));
            Vector::push_back<Event>(&mut notebook.value, newevent);
        }

        // delete event
        public fun deleteEvent(account: &signer, deleteevent:Event) acquires Notebook {
            let notebook = borrow_global_mut<Notebook>(Signer::address_of(account));
            let (result, index) = Vector::index_of(&mut notebook.value, &deleteevent);
            if (result) {
                Vector::remove<Event>(&mut notebook.value,index);
            }
        }

        // change event
        public fun changeEvent(account: &signer, changeevent:Event, newevent:Event) acquires Notebook {
            let notebook = borrow_global_mut<Notebook>(Signer::address_of(account));
            let (result, index) = Vector::index_of(&mut notebook.value, &changeevent);
            if (result) {
                let canchangevent = Vector::borrow_mut<Event>(&mut notebook.value, index);
                let canchangevent = newevent;
            }
        }

        // check all event
        public fun printEvent(account: &signer) acquires Notebook {
            let notebook = borrow_global_mut<Notebook>(Signer::address_of(account));
            let lengthvector = Vector::length(&mut notebook.value);
            if (lengthvector > 0) {
                let i = 1;
                while (i <= lengthvector) {
                    let showevent = Vector::borrow<Event>(&mut notebook.value, i);
                    Debug::print(showevent);
                    i = i + 1
                };
            } else {
                Debug::print(& b"empty notebook");
            }
        }
    } 
}