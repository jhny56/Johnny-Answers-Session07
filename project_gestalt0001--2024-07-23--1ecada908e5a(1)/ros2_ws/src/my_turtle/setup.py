from setuptools import find_packages, setup

package_name = 'my_turtle'

setup(
    name=package_name,
    version='0.0.0',
    packages=find_packages(exclude=['test']),
    data_files=[
        ('share/ament_index/resource_index/packages',
            ['resource/' + package_name]),
        ('share/' + package_name, ['package.xml']),
         ('share/' + package_name + '/launch', ['launch/launch_all_nodes.py']),
    ],
    install_requires=['setuptools'],
    zip_safe=True,
    maintainer='user',
    maintainer_email='user@todo.todo',
    description='TODO: Package description',
    license='TODO: License declaration',
    tests_require=['pytest'],
    entry_points={
        'console_scripts': [
            'stop_on_obstacle = my_turtle.turtle_avoid_wall:main',
            'find_wall_server = my_turtle.wall_finder_server:main',
            'time_lap_server = my_turtle.lap_time_action_server:main',
            'time_lap_client = my_turtle.lap_time_action_client:main'

        ],
    },
)
