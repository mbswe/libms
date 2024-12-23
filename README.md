
## Overview

- **MSMath**: A class that contains mathematical utility functions.
  - **clamp**: A static method to constrain (or clamp) a numeric value between two bounds.


- **MSGeoHash**: A class that contains geohash utility functions.
  - **encode**: A static method to encode a latitude and longitude into a geohash string.
  - **decode**: A static method to decode a geohash string into a latitude and longitude.
  - **distance**: A static method to calculate the distance between two geohash strings. Returns the distance in kilometers.

## Usage
### MSMath    
```MSMath::clamp(mixed $value, mixed $min, mixed $max): int|float```

### MSGeoHash
```MSGeoHash::encode(float $latitude, float $longitude, int $precision = 12): string```

```MSGeoHash::decode(string $hash): array```

```MSGeoHash::distance(string $hash1, string $hash2): float```