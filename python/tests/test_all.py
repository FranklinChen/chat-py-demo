"""
Test creating CHAT JSON from Python and calling Rust library
to get back CHAT text.
"""

import pytest

# Includes Rust stuff.
from chat_py_demo import json_to_chat

# Module generated from JSON Schema for Pydantic.
from chat_py_demo.chat_ast import Chat, Top, TopItem, TopItem1, TopItem2


# TODO
def test_sample_chat() -> None:
    sample_chat = Chat(
        tops=[
            Top(root=TopItem(Header="@Begin")),
            Top(root=TopItem1(MainTier="*CHI:word")),
            Top(root=TopItem2(DependentTier="%com:comment")),
            Top(root=TopItem(Header="@End")),
        ]
    )
    sample_chat_json = sample_chat.model_dump_json()
    expected_chat = """@Begin
*CHI:word
%com:comment
@End
"""
    assert json_to_chat(sample_chat_json) == expected_chat
