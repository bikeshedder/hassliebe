# xtask

Code generators for `hassliebe`. Std-only; not published.

```sh
git clone --depth 1 --filter=blob:none --sparse https://github.com/home-assistant/core.git ha-core
git -C ha-core sparse-checkout set --no-cone '/homeassistant/const.py' \
    '/homeassistant/components/mqtt/' '/homeassistant/components/*/const.py' \
    '/homeassistant/components/*/__init__.py'

cargo xtask generate ha-core   # rewrite all generated regions in src/
cargo xtask check ha-core      # exit 1 if a region is out of date
```

## Regions

Generated code lives between two markers; everything else is hand-written:

```rust
// @generated begin <kind> [args…]
// @generated end
```

The generated text is indented like the begin marker and the whole file is
formatted with `rustfmt`. Kinds:

| Kind | Arguments | Output |
|---|---|---|
| `unit-classes` | – | one closed `string_enum!` per `UnitOf*` class in `homeassistant/const.py`, with `AMBIGUOUS_UNITS` from `sensor/const.py` as aliases |
| `unit-wrapper` | – | `Power(UnitOfPower),` lines for the `Unit` wrapper, sorted (this order is the read precedence) |
| `device-class` | `<path> <PyClass>` | the variants of a `StrEnum`, documented from its docstrings, or from `#` comments if every member without a docstring has one |
| `abbreviations` | `<DICT>` | the entries of a dict in `mqtt/abbreviations.py`, sorted |
| `component` | `<platform>` | imports, struct, builder and `Platform` impl from the spec in `src/platforms.rs` |

## Adding a platform

1. Add a `Platform` to `PLATFORMS` in `src/platforms.rs`. Field docs are short
   paraphrases of the platform's `*.mqtt` docs page.
2. Create `src/component/<platform>.rs` with a module doc, `use
   crate::string_enum::string_enum;` if it defines string enums, an empty
   `component <platform>` region and, per enum, a `string_enum!` with an empty
   `device-class` region.
3. Run `cargo xtask generate <ha-core>`.

Rules the builder should enforce beyond required fields (e.g. "exactly one of
`image_topic` and `url_topic`") go into a hand-written
`fn validate(&self) -> Result<(), String>` on the builder, outside the region;
set `validate: true` in the spec so the struct calls it.

The Python readers are line-based and fail loudly on member lines they do not
understand; adjust `src/python.rs` when Home Assistant changes its style.
