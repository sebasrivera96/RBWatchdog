import requests
import time

DEFAULT_TOKEN = "a2fc903eeaeadf3cbc87cbbdc03ef2d02241217f"
RB_ENDPOINT = "http://127.0.0.1:8000/"
DELAY_BETWEEN_UPDATES = 5
# @TODO: Handle the requests using a short TOKEN
SHORT_TOKEN = ""

def display_initial_message():
    print("===================================================================")
    print("\t\t Welcome to Review Board Watchdog")
    print("===================================================================")

def display_options():
    print("\n===================================================================")
    print("\t Choose an option by typing a character:")
    # Set token
    print("[t] ==> Set Review Board Token")
    # Add a new review
    print("[n] ==> Add a review to be monitored")
    # Begin monitoring
    print("[m] ==> Begin monitoring the reviews!")
    # Exit
    print("[e] ==> Exit")
    return input("===================================================================\n")

def set_token_handler(new_token=""):
    set_token_URL = RB_ENDPOINT + "set_token/"
    if new_token == "":
        print("\n===================================================================")
        new_token = str(input("==> Enter your Review Board token: "))

    if new_token == "default":
        new_token = DEFAULT_TOKEN

    set_token_URL += new_token
    r = requests.get(url=set_token_URL)
    print("Response from the Server: Set Token ==> {}".format(r.text))
    print("===================================================================")

    # @TODO: Implement to return a new short_token
    return new_token

def add_reviewID_handler(new_review_IDs=""):
    add_reviewID_URL = RB_ENDPOINT + "add_review_id/"
    if new_review_IDs == "":
        print("\n===================================================================")
        new_review_IDs = str(input("==> Enter the new Review IDs (comma separated): "))
    reviews_list = new_review_IDs.split(',')
    for new_review in reviews_list:
        r = requests.get(url=add_reviewID_URL + new_review)
        print("Response from the Server: Add ReviewID ==> {}".format(r.text))
    print("===================================================================")

def begin_monitoring_handler():
    quit_monitoring = ''
    get_updates_URL = RB_ENDPOINT + "get_updates/"
    while quit_monitoring != 'q':
        start_time = time.time()

        while time.time() - start_time < DELAY_BETWEEN_UPDATES:
            # quit_monitoring = input()
            pass

        raw_updates = requests.get(url=get_updates_URL)
        display_updates(raw_updates)

def display_updates(raw_updates):
    json_updates = raw_updates.json()
    updates_available = False
    for review_info in json_updates['review_ids']:
        if review_info['new_update_available']:
            updates_available = True
            break
    if updates_available:
        print("\n===================================================================")
        for review_info in json_updates['review_ids']:
            if review_info['new_update_available'] == True:
                print("\n\tNEW UPDATE ON Review ==> {}".format(review_info['review_id']))
                print("\tUpdate's USERNAME ==> {}".format(review_info['username']))
                print("\tSUMMARY ==> {}\n".format(review_info['summary']))
        print("===================================================================\n")

def test1():
    set_token_handler(new_token="default")
    add_reviewID_handler(new_review_IDs="311878,304955")
    begin_monitoring_handler()

if __name__ == "__main__":
    current_option = ''
    current_token = ""

    display_initial_message()
    while current_option != 'e':
        current_option = display_options()
        # Call the handlers
        if current_option == 'e':
            break
        elif current_option == '1':
            test1()
        elif current_option == 't':
            current_token = set_token_handler()
        elif current_option == 'n':
            add_reviewID_handler()
        elif current_option == 'm':
            begin_monitoring_handler()
        else:
            print("\n########## THERE IS NO OPTION ==> {} ###########\n".format(current_option))
