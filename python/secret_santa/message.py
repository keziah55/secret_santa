import smtplib
import ssl
from email.mime.multipart import MIMEMultipart
from email.mime.text import MIMEText
from string import Template


class MessageSender:
    """Class to send emails."""

    def __init__(self, sender: dict):
        self._sender = sender

        self._msg = Message()

    def send_msg(self, subject: str, to_name: str, to_address: str, giftee_name: str):
        """Send email with `subject` and optional attachment."""

        if not to_address:
            raise ValueError(f"No email address given for {to_name}")

        msg = MIMEMultipart("alternative")
        msg["Subject"] = subject
        msg["From"] = self._sender["address"]
        msg["To"] = to_address

        content = self._msg.make_message(santa=to_name, recipient=giftee_name)
        msg.attach(MIMEText(content, "html"))
        self._send(msg)

    def _send(self, msg: MIMEMultipart):
        context = ssl.create_default_context()

        with smtplib.SMTP_SSL(
            self._sender["smtp"], self._sender["port"], context=context
        ) as server:
            server.login(self._sender["address"], self._sender["cred"])
            server.send_message(msg)


class Message:

    HTML_HEADER = Template(
        """
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>${title}</title>
</head>  
"""
    )

    HTML_FOOTER = "</html>"

    MSG_TITLE = "Secret Santa!"

    @classmethod
    def make_message(cls, santa: str, recipient: str, char: str = "#") -> str:

        ascii_img = [
            "",
            f" .-.   Dear {santa},",
            "(o o) I am the ghost of Christmas presents,",
            "| O \\  and your secret santa is...",
            f" \\   \\  {recipient.upper()}!",
            "  `~~~'",
            "",
        ]

        start = f"{char} "
        end = f" {char}"
        width = max(len(line) for line in ascii_img) + len(start) + len(end)

        for n, line in enumerate(ascii_img):
            if len(line) == 0:
                ascii_img[n] = char * width
            else:
                line_start = f"{start}{line}"
                pad = width - len(line_start) - len(char) - 1
                ascii_img[n] = f"{line_start}{" "*pad}{end}"

        header = cls.HTML_HEADER.substitute(title=cls.MSG_TITLE)

        tag = "code"

        return "\n".join(
            [header, "<body>", f"<{tag}>"]
            + ascii_img
            + [f"</{tag}>", "</body>", cls.HTML_FOOTER]
        )
