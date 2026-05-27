package internal

import "math"

var basePlotPrice int = 100
var plotPriceGrowth float64 = 1.5

func nextPlotPrice(plotCount int) int {
	return int(float64(basePlotPrice) * math.Pow(plotPriceGrowth, float64(plotCount-3)))
}
