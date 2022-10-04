package cars

// CalculateWorkingCarsPerHour calculates how many working cars are
// produced by the assembly line every hour.
func CalculateWorkingCarsPerHour(productionRate int, successRate float64) float64 {
	return float64(productionRate) * .01 * successRate
}

// CalculateWorkingCarsPerMinute calculates how many working cars are
// produced by the assembly line every minute.
func CalculateWorkingCarsPerMinute(productionRate int, successRate float64) int {
	return int(CalculateWorkingCarsPerHour(productionRate, successRate)) / 60
}

// CalculateCost works out the cost of producing the given number of cars.
func CalculateCost(carsCount int) uint {
	if carsCount >= 10 {
		batchesOfTen := carsCount / 10
		remainder := carsCount - (batchesOfTen * 10)

		bundled_cost := batchesOfTen * 95000
		remainder_cost := remainder * 10000

		return uint(remainder_cost) + uint(bundled_cost)
	} else {
		return uint(carsCount) * 10000
	}

}
