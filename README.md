
# Конфигурация

Параметры задаем  в `config.toml`.

## API

GET: Получение записи с указанным id

```bash
curl -X GET "http://localhost:8080/get_order?order_uid=30
```

POST: Создание новой записи

```bash
curl -X POST http://localhost:3000/api/orders/save_order \
     -H "Content-Type: application/json" \
     -d '{"keys": "values"}'
```

## Запуск сервера

```bash
cargo run --bin server localhost:8080
```

## Запуск скрипта

### Получение ордера

```bash
  cargo run --bin script get 2
```
### Отправка ордера

```bash
  cargo run --bin script post ./model.json 
```
