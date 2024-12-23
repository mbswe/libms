## Classes and Methods

### MSMath
A class that contains mathematical utility functions.

- **clamp**: Constrain (or clamp) a numeric value between two bounds.
  ```php
  MSMath::clamp(mixed $value, mixed $min, mixed $max): int|float
  ```

- **lerp**: Linearly interpolate between two values.
  ```php
  MSMath::lerp(float $start, float $end, float $t): float
  ```

- **inverseLerp**: Calculate the inverse linear interpolation of a value between two bounds.
  ```php
  MSMath::inverseLerp(float $start, float $end, float $value): float
  ```

- **remapRange**: Remap a value from one range to another.
  ```php
  MSMath::remapRange(float $value, float $oldMin, float $oldMax, float $newMin, float $newMax): float
  ```

- **mean**: Calculate the mean of an array of numbers.
  ```php
  MSMath::mean(array $values): float
  ```

### MSGeoHash
A class that contains geohash utility functions.

- **encode**: Encode a latitude and longitude into a geohash string.
  ```php
  MSGeoHash::encode(float $latitude, float $longitude, int $precision = 12): string
  ```

- **decode**: Decode a geohash string into a latitude and longitude.
  ```php
  MSGeoHash::decode(string $hash): array
  ```

- **distance**: Calculate the distance between two geohash strings. Returns the distance in kilometers.
  ```php
  MSGeoHash::distance(string $hash1, string $hash2): float
  ```

- **neighbor**: Calculate the neighbor of a geohash string in a given direction (N, S, E, W, NW...).
  ```php
  MSGeoHash::neighbor(string $hash, string $direction): string
  ```

- **neighbors**: Calculate the neighbors of a geohash string in all directions (N, S, E, W, NW...).
  ```php
  MSGeoHash::neighbors(string $hash): array
  ```

### MSMisc
A class that contains miscellaneous functionality.

- **toExcelColumn**: Convert a number to an Excel column name.
  ```php
  MSMisc::toExcelColumn(int $number): string
  ```