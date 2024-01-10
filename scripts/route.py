import sys

route = sys.argv[1]
component = f"{route.capitalize()}Route"

print(f"{{ path: '{route}', component: {component} }},")
