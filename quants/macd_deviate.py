import talib

def doDecide(stock, grid):
	# import jqdatasdk
	# grid = jqdatasdk.attribute_history(stock, rows, fields=['close']).dropna()

	fast = 11
	slow = 26
	sign = 5
	rows = (fast + slow + sign) * 5
	suit = {'dif':0, 'dea':0, 'macd':0, 'gold':False, 'dead':False}

	grid['dif'], grid['dead'], grid['macd'] = talib.MACD(grid['close'].values, fast, slow, sign)
	grid = grid.dropna()

	# 底背离----------------------------------------------------------------
	mask = grid['macd']>0
	mask = mask[mask==True][mask.shift(1)==False]
	key2 = mask.keys()[-2]
	key1 = mask.keys()[-1]
	suit['gold'] = grid.close[key2] > grid.close[key1] and grid.dif[key2] < grid.dif[key1] < 0 and grid.macd[-2] < 0 <grid.macd[-1]

	# 顶背离----------------------------------------------------------------
	mask = grid['macd']<0
	key2 = mask.keys()[-2]
	key1 = mask.keys()[-1]
	suit['dead']= grid.close[key2] < grid.close[key1] and grid.dif[key2] > grid.dif[key1] > 0 and grid.macd[-2] > 0 >grid.macd[-1]

	return suit
