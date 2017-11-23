# AWS EC2 Setup (Optional)

## Dependencies

* AWS CLI

## Setup on Local Machine
You can use a AWS EC2 instance as remote build machine. Simple define `ec2_instance_id`
in the `config` (see [local setup](SETUP_LOCAL.md)) with the instance id to use. Install and setup AWS CLI using:

```
brew install awscli
aws configure
```

Mainframer will use your gloabl AWS configuration, so make sure to use the correct region
and secret key for your EC2 instance when configuring AWS CLI.

## Recommended Setup on AWS
It is strongly recommended to use a Cloud Watch alarm to automatically shutdown the
EC2 instance when not in use. Mainframer will automatically start the instance when
a build is started but the EC2 instance is not running.

*Disclaimer*: Running a t2.large instance will cost you roughly $ 0.10 per hour. Look [here](https://aws.amazon.com/en/ec2/pricing/) for details.

Perform following steps in AWS:

0. Create a new AWS account if not done yet and add a new payment method
1. Go to `Services` -> `EC2` -> `Instances` -> `Launch Instance`
2. Select Amazon Linux (SSD Volume Type)
3. Select a instance type. It's recommended to use a t2.* instance as they are able to provide higher performance in bursts which is usually the case when developing and compiling every now and then. Using a t2.large instance is a good point of start, you then can either go up and down the instance types based on your needs (you also can change the instance later).
4. Click `Review and Launch` and then `Launch`
5. Create a new key pair. Store the `.pem` file downloaded at `~/.ssh/` and use it as `{SSH_KEY_NAME}` in [the local setup](SETUP_LOCAL.md)
6. In the list of instances, select the newly created instance
7. Select the `Monitoring` tab and then `Create Alarm`
8. Uncheck `Send a notification to` and check `Take action`and `Stop this instance`
9. Select `Whenever Maximum of CPU Ultilization Is <= 5 Percent For at least 6 consecutive period(s) of 5 Minutes`
10. Click `Create Alarm`
11. Go to `Elastic IPs` and select `Allocate new address`, then `Allocate` and then `Close`
12. Right-click on the address in the list and click `Associate address`
13. Select your instance EC2 instance in `Instance`
14. Select a private IP in `Private IP`
15. Click `Associate`
16. Copy the IP address and use it ad `{REMOTE_MACHINE_IP_OR_HOSTNAME}` in [the local setup](SETUP_LOCAL.md)
