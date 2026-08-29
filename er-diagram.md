```mermaid
erDiagram
    walks {
        Guid id PK
        string name
        string description "Nullable"
        double length_km
        string image_url "Nullable"
        Guid region_id FK
        difficulty difficulty "Easy | Medium | Hard"
    }

    regions {
        Guid id PK
        string code
        string name
        string image_url "Nullable"
    }

    walks }o--|| regions: "walks belongs to region"
```