# libspinners

[![](https://img.shields.io/github/v/tag/thechampagne/libspinners?label=version)](https://github.com/thechampagne/libspinners/releases/latest) [![](https://img.shields.io/github/license/thechampagne/libspinners)](https://github.com/thechampagne/libspinners/blob/main/LICENSE)

Elegant terminal spinners for **C**.

### Installation & Setup

#### 1. Clone the repository
```
git clone https://github.com/thechampagne/libspinners.git
```
#### 2. Navigate to the root
```
cd libspinners
```
#### 3. Build the project
```
cargo build
```

### API

```c
typedef struct spinner_t spinner_t;

typedef enum {
	...
} spinner_spinners_t;

spinner_t spinner_new(spinner_spinners_t spinner, const char* message);

spinner_t spinner_new_with_timer(spinner_spinners_t spinner, const char* message);

void spinner_stop(spinner_t* spinner);

void spinner_stop_with_symbol(spinner_t* spinner, const char* symbol);

void spinner_stop_with_newline(spinner_t* spinner);

void spinner_stop_with_message(spinner_t* spinner, const char* msg);

void spinner_stop_and_persist(spinner_t* spinner, const char* symbol, const char* msg);

void spinner_clean(spinner_t* spinner);
```

### References
 - [spinners](https://github.com/FGRibreau/spinners)

### License

This repo is released under the [MIT](https://github.com/thechampagne/libspinners/blob/main/LICENSE).
