import json

# 입력 파일 경로 설정
input_file = 'commands.sh'

# validator 함수 정의
def validate_token_ids(commands_file):
    with open(commands_file, 'r') as file:
        lines = file.readlines()

    for line_number, line in enumerate(lines, start=1):
        # JSON 문자열 추출
        json_str = line.split("sei1ycqlpfxtm806jgx54j7kq352tf3443wdwrl6e2kfu964zu6rzuuqujnd2l '")[1].split("'")[0]

        # JSON 파싱
        try:
            json_data = json.loads(json_str)
        except json.JSONDecodeError as e:
            print(f"Error decoding JSON on line {line_number}: {e}")
            continue

        # token_id 확인
        token_id = json_data.get('mint', {}).get('token_id')
        if token_id is None:
            print(f"Error on line {line_number}: token_id is missing")
            continue
        try:
            token_id_int = int(token_id)
            if token_id_int < -2 or token_id_int > 2099:
                print(f"Error on line {line_number}: token_id must be between 0 and 2099")
        except ValueError:
            print(f"Error on line {line_number}: token_id must be an integer")

# validator 함수 호출
validate_token_ids(input_file)
