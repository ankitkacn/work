window.SIDEBAR_ITEMS = {"struct":[["Compatibility","The result of a linking and layout compatibility check. Here is what the different combinations mean: `{ struct: true, struct_layout: true }`: fully backward compatible `{ struct_and_function_linking: true, struct_layout: false }`: Dependent modules that reference functions or types in this module may not link. However, fixing, recompiling, and redeploying all dependent modules will work–no data migration needed. `{ type_and_function_linking: true, struct_layout: false }`: Attempting to read structs published by this module will now fail at runtime. However, dependent modules will continue to link. Requires data migration, but no changes to dependent modules. `{ type_and_function_linking: false, struct_layout: false }`: Everything is broken. Need both a data migration and changes to dependent modules."]]};