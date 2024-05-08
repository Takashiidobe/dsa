from scipy.stats import chi2

# Number of outcomes and total tests
num_outcomes = 24
total_tests = 100000

# Expected frequency for each outcome
expected_frequency = total_tests / num_outcomes

# Critical value for 23 degrees of freedom at 95% confidence
critical_value = chi2.ppf(0.05, num_outcomes - 1)

print(expected_frequency, critical_value)
