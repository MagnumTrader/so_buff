# so_buff

Made by me for learning.\
Does less things than [Arrayvec](https://docs.rs/arrayvec/latest/arrayvec/), \
You should probably use Arrayvec instead.

Made for my own use case where i need a buffer that gets passed and then consumed.
No pushing after you have started consuming.
Could be achieved with the new type pattern over an Arrayvec but where is the fun in that? :)

Benches tell me to use a standard Vec when 100 >= items.
so_buff::Buffer is faster on pushing and consuming 10 items, so may have its use cases.




