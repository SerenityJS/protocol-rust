Consideration: ClassInstances in napi act odd in JavaScript. Possibly hack in a way to apply method calls on napi::JsObject's.
               or create a proc macro which will apply to_object methods to classes to they can be converted to generic format.

Consideration: ClassInstances don't allow mutation of sub ClassInstances nor does it allow mutation of Arrays and other notorious
               types. Possibly napi::JsObject's don't act the same way?

Consideration: BinaryStream is less the sub-optimal figure out what works better once more packets are created. Ideally seperate
               methods into seperate endianess again. Possibly infer conversions from types; Expose through napi?

Consideration: 