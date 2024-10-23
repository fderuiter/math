import numpy as np
from scipy.integrate import quad, dblquad
import random

# Constants and parameters
T = 365
g_emotional = 5
g_practical = 2
f_initial = 7
birth_order_weight = 1.2
major_life_events = 3
helped_during_crisis = True
H = 1.5 if helped_during_crisis else 1
active_on_social_media = True
S = 1.3 if active_on_social_media else 1
decay_constant = 0.05
time_since_last_contact = 7
D = np.exp(-decay_constant * time_since_last_contact)
intelligence = 7
emotional_sensitivity = 6
wealth = 9
talent = 8
w_I = 1.2
w_Es = 1.5
w_W = 1.1
w_T = 1.3
sibling_distances = [100, 50, 10]
compliments = {'cooking': 10, 'appearance': 5, 'intelligence': 8}
compliment_weights = {'cooking': 1, 'appearance': 0.5, 'intelligence': 0.75}
R = random.uniform(0.9, 1.1)
x_0 = 20

# Functions for integrals
def proximity(t):
    return 1 / x_0

def emotional_support(x, t):
    return 8

def sibling_proximity(t):
    return sum(1 / distance for distance in sibling_distances)

# Calculations
proximity_integral = quad(proximity, 0, T)[0]
emotional_support_integral = dblquad(emotional_support, 0, T, lambda t: 0, lambda t: 1)[0]
gift_matrix = np.array([[g_emotional, 0], [0, g_practical]])
gift_matrix_determinant = np.linalg.det(gift_matrix)
compliment_values = np.array([compliments[key] for key in compliment_weights])
compliment_weight_values = np.array([compliment_weights[key] for key in compliment_weights])
compliment_score = np.dot(compliment_values, compliment_weight_values)
frequency_term = np.log(1 + f_initial)
personality_score = w_I * intelligence + w_Es * emotional_sensitivity + w_W * wealth + w_T * talent
sibling_proximity_integral = quad(sibling_proximity, 0, T)[0]

numerator = (
    proximity_integral *
    emotional_support_integral *
    gift_matrix_determinant *
    compliment_score *
    frequency_term *
    personality_score *
    birth_order_weight *
    major_life_events *
    H *
    S *
    D *
    R
)

denominator = sibling_proximity_integral

F = numerator / denominator

print(f"Favoritism Score: {F}")
