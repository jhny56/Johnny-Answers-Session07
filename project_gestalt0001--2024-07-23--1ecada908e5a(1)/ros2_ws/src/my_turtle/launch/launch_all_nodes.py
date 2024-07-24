from launch import LaunchDescription
from launch_ros.actions import Node

def generate_launch_description():
    return LaunchDescription([
        Node(
            package='my_turtle',
            executable='find_wall_server',
            name='find_wall_server',
            output='screen'
        ),
        Node(
            package='my_turtle',
            executable='stop_on_obstacle',
            name='stop_on_obstacle',
            output='screen'
        ),
        Node(
            package='my_turtle',
            executable='time_lap_server',
            name='time_lap_server',
            output='screen'
        ),
        Node(
            package='my_turtle',
            executable='time_lap_client',
            name='time_lap_client',
            output='screen'
        ),
    ])
