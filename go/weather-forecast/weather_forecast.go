// Package weather provides a forecast for a specified location.
package weather

// CurrentCondition is the current weather condition at the specified location.
var CurrentCondition string

// CurrentLocation is the location where the weather should be checked.
var CurrentLocation string

// Forecast returns the upcoming weather conditions.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
