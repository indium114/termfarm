package internal

import "math"

const basePlotPrice int = 100
const plotPriceGrowth float64 = 1.5

func nextPlotPrice(plotCount int) int {
	return int(float64(basePlotPrice) * math.Pow(plotPriceGrowth, float64(plotCount-3)))
}
