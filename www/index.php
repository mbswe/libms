<?php

// MSMath examples
echo "Clamp: " . MSMath::clamp(15, 10, 20) . "<br>";
echo "Lerp: " . MSMath::lerp(0.0, 10.0, 0.5) . "<br>";
echo "Inverse Lerp: " . MSMath::inverseLerp(0.0, 10.0, 5.0) . "<br>";
echo "Remap Range: " . MSMath::remapRange(5.0, 0.0, 10.0, 0.0, 100.0) . "<br>";
echo "Mean: " . MSMath::mean([1, 2, 3, 4, 5]) . "<br>";

// MSGeoHash examples
echo "Encode: " . MSGeoHash::encode(37.7749, 22.4194) . "<br>";
print_r(MSGeoHash::decode("9q8yy"));
echo "<br>Distance: " . MSGeoHash::distance("9q8yy", "9q8yz") . "<br>";
echo "Neighbor: " . MSGeoHash::neighbor("9q8yy", "N") . "<br>";
print_r(MSGeoHash::neighbors("9q8yy"));
echo "<br>";

// MSMisc example
echo "To Excel Column: " . MSMisc::toExcelColumn(28) . "<br>";