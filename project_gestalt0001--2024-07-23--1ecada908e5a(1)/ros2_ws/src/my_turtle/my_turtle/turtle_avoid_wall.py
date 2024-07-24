import rclpy
from rclpy.node import Node
from geometry_msgs.msg import Twist
from sensor_msgs.msg import LaserScan
from my_interfaces.srv import FindWall
from std_msgs.msg import Bool


class RobotDriver(Node):

    def __init__(self):
        super().__init__('robot_driver')
        self.publisher_ = self.create_publisher(Twist, '/cmd_vel', 10)
        self.subscription = self.create_subscription(LaserScan, '/scan', self.listener_callback, 10)
        self.distance_to_wall = 0.2  # desired distance to the wall in meters
        self.wall_distance_tolerance = 0.05  # tolerance in meters
        self.distance_threshold = 0.3 # distance threshold for detecting obstacles in meters
        self.wall_found_publisher = self.create_publisher(Bool, 'wall_found', 10)

        # Wall-finding state
        self.finding_wall = True

        self.client = self.create_client(FindWall, 'find_wall')
        while not self.client.wait_for_service(timeout_sec=1.0):
            self.get_logger().info('Service not available, waiting...')

        

    def listener_callback(self, msg):
        twist = Twist()

        if self.finding_wall:
            # Finding the wall logic
            self.find_wall_logic(msg, twist)
        else:
            # Following the wall logic
            
            self.follow_wall_logic(msg, twist)

        # Publish the command to the /cmd_vel topic
        self.publisher_.publish(twist)

    def find_wall_logic(self, msg, twist):
        # Logic to find the wall
        ranges = msg.ranges
        front_region = min(min(ranges[0:20] + ranges[-20:]), 10)

        if front_region < self.distance_threshold:
            twist.linear.x = 0.0
            twist.angular.z = 0.5  # Turn to find the wall
            self.finding_wall = False
            self.wall_found_publisher.publish(Bool(data=True))
            self.get_logger().info(f'WALL FOUND : DATA = TRUE')

        else:
            twist.linear.x = 0.2
            twist.angular.z = 0.0  # Move forward to find the wall

    def follow_wall_logic(self, msg, twist):
        # Logic to follow the wall
        ranges = msg.ranges
        front_region = min(min(ranges[0:20] + ranges[-20:]), 10)
        right_region = min(min(ranges[-90:-60]), 10)  # Adjust indices based on sensor configuration

        if front_region < self.distance_threshold:
            twist.linear.x = 0.0
            twist.angular.z = 0.5  # Turn left to avoid front obstacle
        else:
            if right_region < self.distance_to_wall - self.wall_distance_tolerance:
                twist.linear.x = 0.4
                twist.angular.z = 0.2  # Too close to wall, turn left
            elif right_region > self.distance_to_wall + self.wall_distance_tolerance:
                twist.linear.x = 0.4
                twist.angular.z = -0.2  # Too far from wall, turn right
            else:
                twist.linear.x = 0.4
                twist.angular.z = 0.0  # Move forward while maintaining distance

def main(args=None):
    rclpy.init(args=args)
    node = RobotDriver()

    node.req = FindWall.Request()
    node.future = node.client.call_async(node.req)
    rclpy.spin_until_future_complete(node, node.future)
    node.get_logger().info('Found the wall')
    node.finding_wall = False  # Change state to wall-following

    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    node.destroy_node()
    rclpy.shutdown()

if __name__ == '__main__':
    main()


# ros2 launch turtlebot3_gazebo turtlebot3_dqn_stage1.launch.py
# export TURTLEBOT3_MODEL=burger