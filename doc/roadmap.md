# Abstrasim

AbsTraSim (Absurd Trading Simulator) is a game where the player tries to make
money in a fictional and unrealistic world by trading simulated stocks.

## The Economy

The economy in AbsTraSim is very far from the real world economy. For simplification, we consider that:

- There is only one global and worldly stock exchanges. The W$ (world dollar) is the universal currency.

- For the moment, let us assume that there are no regional differences in the world and that all countries
have similar economies.

- The World Bank is now the central bank for the whole world and gets to decide what interest rates it applies.
Changing this rates DO have an effect (which is probably not that absurd).

### Industries

For the time being, there are 13 sectors.

* Oil & Energy
* Mining
* Food & agriculture
* Pharmaceutics
* Machine tool
* Automobile
* Luxury
* Software
* Logistics
* Tourism
* Media
* Telecom
* Banking
* Distribution

Sectors can influence each other.

Oil & Energy -> Logistics, Tourism
Mining -> Machine tool, automobile
Food & agriculture -> Pharmaceutics, Luxury
Logistics, Banking -> Every sectors

### Businesses

Each industry sector have between 10 and 20 businesses; these are the top
players, worldwide, financially.

The basic game mechanism is to make money by trading those stocks, so one of the
main aspect should be computing how the stocks evolve.

Stock evolution "in the real world" are ultimately dependant on *the general
perception of the market". Which makes it difficult to simulate, since the ideal
way would be to take the thousands of investors, and simulate their own
rationale and predictions. We will use, more reasonably, a mix of general
context and random.

The main aspect though is the difference between how the business really
performs and how it is perceived to perform. Part of the gameplay should be
being able to see what businesses are over or underrated.

Businesses should have the following attributes notes, as real from 0 to 1.

* Leadership : quality of management & CEO.
* Size : sheer size of the company - should influence its stock volatility and
  marketshare, by determining the shares outstanding.
* Investement : does the company invest a lot ? If so, it might emit new stocks.
  If not, it will be more stable, but tend to lose in size.
* PR : ability of the company to manipulate perception of its worth.
* Results : semi-random value, changed by leadership and investement, that
  indicates how well the company is performing.
* Perception: how accurately the market, generally, perceives this company.

Each company should have yearly events deciding its administration and its result publication.

Each day, events might affect companies : crisis, investigations, announcement, new management, etc.

Finally, each company has of course a stock value, shares outstanding and market
capitalization that will vary every hour depending on the perception of all those parameters.

Perception is mostly random : every time we want to compute the value of a stock, we actually want
to compute the difference between real performance and perceived performance.

Computing the performance is easy: 
    * Avg of leadership + size + investement + results + events + sectorial health

That gives you a float between 0 and 1.

Computing the perception is more difficult :
    * Avg Random factor + PR + real performance

That also gives you a float between 0 and 1.

Perceived performance is now the average of real performances and perception.
If perceived performance is higher than last perceived performance, more people will want to buy.
If perceived performance is lower than last perceived performance, more people will want to sell.
This give us the "general tendancy" or G.

     G = perception - performance

     Example : business has a real performance of .7
     Last perception was .57
     New perception is .62
     So a + .05 difference

The real evolution of the stock, however, will depend on the shares outstanding; we shall consider
that a lowing volume create more volatility. So, the evolution of the stock value will depend 
on an intermediary value called "volume factor", or V.

    V = ABS ( ( 1 / millions of shares outstanding) * G * 1000 )
    
    Example : on the former business, if the outstanding shares value is (in million) 500,
    this gives us abs( 1/500 ) * .05 * 1000, so 0.1

Volume of transaction / day can be seen as outstanding shares * this value.

    In this example, there would be a volume of 30 million, which is not bad for a business with
    300 million outstanding shares !

The market capitalization will evolve by a number E depending on G and V, as such :

    E = previous capitalization * G * V

    In this example, if the market capitalization was 3 billion (so 10$ a share), it would change
    by 300 * 0.5 * 0.1, so 15 millions (which means), so a .5% change.
