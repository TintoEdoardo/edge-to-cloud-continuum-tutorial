
;; +-- PART 1 --+
;; Interfacing with the host environment

;; A module has but two interfaces with the
;; hosting environment.
;; One consists of imported/exported functions,
;; the other is its linear memory.

;; Import and export function.
(module $the_module

    ;; Here the module declares a host-defined
    ;; function.
    (import "the_host" "print" (func $print_to_host (param i32)))

    ;; This is a function without parameters
    ;; and without a return value.
    (func $main_function
        i32.const 15

        ;; Function invocation.
        call $print_to_host

    )

    (export "main_function" (func $main_function))

)