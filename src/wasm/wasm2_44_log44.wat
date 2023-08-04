(module
 (type $FUNCSIG$vi (func (param i32)))
 (import "env" "import_log" (func $import_log (param i32)))
 (table 0 anyfunc)
 (memory $0 1)
 (export "memory" (memory $0))
 (export "main" (func $main))
 (func $main (; 1 ;) (result i32)
  (call $import_log
   (i32.const 44)
  )
  (i32.const 44)
 )
)
