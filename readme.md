# Stack Overflow Clone

## Building

```shell
cargo watch -q -c -w src/ -x run
```

## Table structure

### Question

| Name          | Type         | Description                                  |
| ------------- | ------------ | -------------------------------------------- |
| question_uuid | UUID         | Generated identifier unique to each question |
| title         | VARCHAR(255) | Title of the question                        |
| description   | VARCHAR(255) | Description of the question                  |
| created_at    | TIMESTAMP    | Creation timestamp of the question           |

### Answer

| Name          | Type         | Description                                  |
| ------------- | ------------ | -------------------------------------------- |
| answer_uuid   | UUID         | Generated identifier unique to each answer   |
| question_uuid | UUID         | Generated identifier unique to each question |
| content       | VARCHAR(255) | Content of the answer                        |
| created_at    | TIMESTAMP    | Creation timestamp of the answer             |

## **API (endpoints & models)**

### Questions

#### **Question creation**

> POST /question

Sample request

```shell
curl --request POST \
  --url http://localhost:8000/question \
  --header 'Accept: application/json' \
  --data '{
    "title": "Newly Created Question",
    "description": "My Description"
  }'
```

Sample response

```json
{
  "question_uuid": "d347261c-3f0e-42d2-8706-5ef9f1b96725",
  "title": "Newly Created Question",
  "description": "My Description",
  "created_at": "2024-01-01 00:00:00.000000"
}
```

---

#### **Question retrieval**

> GET /questions

Sample request

```shell
curl --request GET \
  --url http://localhost:8000/questions \
  --header 'Accept: application/json'
```

Sample response

```json
[
  {
    "question_uuid": "d347261c-3f0e-42d2-8706-5ef9f1b96725",
    "title": "Newly Created Question",
    "description": "My Description",
    "created_at": "2024-01-01 00:00:00.000000"
  }
]
```

---

Question deletion

> DELETE /question

Sample request

```shell
curl --request DELETE \
  --url http://localhost:8000/question \
  --header 'Accept: application/json' \
  --data '{
    "question_uuid": "b068cd2f-edac-479e-98f1-c5f91008dcbd"
  }'
```

Sample response

`HTTP 200 OK`

---

### Answers

#### **Answer creation**

> POST /answer

Sample request

```shell
curl --request POST \
  --url http://localhost:8000/answer \
  --header 'Accept: application/json' \
  --data '{
    "question_uuid": "b068cd2f-edac-479e-98f1-c5f91008dcbd",
    "content": "test question"
  }'
```

Sample response

```json
{
  "answer_uuid": "a1a14a9c-ab9e-481b-8120-67f675531ed2",
  "question_uuid": "b068cd2f-edac-479e-98f1-c5f91008dcbd",
  "content": "test question",
  "created_at": "2024-01-01 00:00:00.000000"
}
```

---

#### **Answer retrieval**

> GET /answers

Sample request

```shell
curl --request GET \
  --url http://localhost:8000/answers \
  --header 'Accept: application/json' \
  --data '{
    "question_uuid": "b068cd2f-edac-479e-98f1-c5f91008dcbd"
  }'
```

Sample response

```json
[
  {
    "answer_uuid": "a1a14a9c-ab9e-481b-8120-67f675531ed2",
    "question_uuid": "b068cd2f-edac-479e-98f1-c5f91008dcbd",
    "content": "test question",
    "created_at": "2024-01-01 00:00:00.000000"
  }
]
```

---

#### **Answer deletion**

> DELETE /answer

Sample request

```shell
curl --request DELETE \
  --url http://localhost:8000/answer \
  --header 'Accept: application/json' \
  --data '{
    "answer_uuid": "a1a14a9c-ab9e-481b-8120-67f675531ed2"
  }'
```

Sample response

`HTTP 200 OK`

---

## Objectives

- Designing & building APIs
- Using a backend framework (`Rocket`)
- Using a database (`PostgreSQL`)
- Using third-party crates (`tokio`)
- Writing testable code (`TDD`)
- Organizing code using modules
- Navigating and contributing to an existing code base
